import joblib
import flask
import numpy as np
import os
import requests
import json
import datetime
import pytz
import exeWhatMusic
import aiNangoQa
import psycopg2
import topNangoAICom

#ポート番号
TM_PORT_NO = 7017

# initialize our Flask application and pre-trained model
app = flask.Flask(__name__)
app.config['JSON_AS_ASCII'] = False  # <-- 日本語の文字化け回避


DATABASE_URL = "postgresql://postgres:postgres@postgres:5432/webque"

# DBから取得した文章データリスト
global sentences

@app.route('/alexspeak/api/how-to-spend', methods=['GET'])
def get_how_to_spend():
    alexSpeakInfos = getAlexSpeakMojiRst()
    return flask.jsonify({'alexSpeakInfos': alexSpeakInfos})


# 占い結果を返す
def getUranaiRst():
    now_jp = datetime.datetime.now(pytz.timezone('Asia/Tokyo'))
    date_jp_form = now_jp.strftime("%Y/%m/%d")

    res = requests.get(url='http://api.jugemkey.jp/api/horoscope/free/'+ date_jp_form)
    uranaiRstKaniza = res.json()["horoscope"][date_jp_form][3]["content"]
    
    return uranaiRstKaniza

# 天気予報の結果を返す
def getWeatherInfo():
    LOCATION = 'Tokyo,jp' # 場所を設定します。
    APPID='de8940f9f25cc75800cd17380cd25ef8' # openweathermap のAPIキーを設定してください

    # 天気のデータを取得する
    # url ='http://api.openweathermap.org/data/2.5/forecast?q={}&cnt=10&appid={}&units=metric'\
    url ='http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric'\
    .format(LOCATION, APPID)
    response = requests.get(url)
    response.raise_for_status()

    weather_data = json.loads(response.text)
    
    # w = weather_data['list']
    # tenki_1200 = w[2]['weather'][0]['main']
    # temp_1200 = w[2]['main']['temp']

    tenki_1200 = weather_data['weather'][0]['main']
    temp_1200 = weather_data['main']['temp']

    # 雨かどうか
    isRain = False
    if tenki_1200 == 'Rain':
        isRain = True
    else:
        isRain = False
        
    # 服装を返す
    fukuso = '冬服'
    if -80.0 < temp_1200 < 10.0:
        fukuso = '冬服'
    elif 10.0 <= temp_1200 < 15.0:
        fukuso = '秋服'
    elif 15.0 <= temp_1200 < 25:
        fukuso = '春服'
    elif 25 <= temp_1200:
        fukuso= '夏服'
    
    return isRain, fukuso

# DBへのConectionを取得します。
def getDbConnection():
    return psycopg2.connect(DATABASE_URL)

# DBから文書データを取得します。
def getSentenceFromDB(con):
    infos = []
    with con.cursor() as cur:
        cur.execute("SELECT * FROM sentence_str WHERE del_flag = false ORDER BY id ASC")
        for row in cur:
            infos.append(row)
    return infos

# 取得時間情報を元に文字列を返却します。
def getDateInfoStr():
    now_jp = datetime.datetime.now(pytz.timezone('Asia/Tokyo'))
    hourNow = now_jp.hour

    global sentences
    with getDbConnection() as con:
        sentences = getSentenceFromDB(con)

    # 挨拶文字と応援フレーズを生成
    greetingStr = ''
    cheerPhrase = ''
    if 5 < hourNow < 10:
        greetingStr = sentences[0][1]
        cheerPhrase = sentences[0][2]
    elif 10 <= hourNow < 16:
        greetingStr = sentences[1][1]
        cheerPhrase = sentences[1][2]
    elif 16 <= hourNow < 21:
        greetingStr = sentences[2][1]
        cheerPhrase = sentences[2][2]
    elif 21 <= hourNow < 24:
        greetingStr = sentences[3][1]
        cheerPhrase = sentences[3][2]
    else:
        greetingStr = sentences[4][1]
        cheerPhrase = sentences[4][2]

    # 今日の残り時間文字列を生成
    time_jp_str = now_jp.strftime('%H:%M:%S')
    nowTimeJp = datetime.datetime.strptime(time_jp_str, '%H:%M:%S')
    oneDayHour = datetime.datetime.strptime('23:59:59', '%H:%M:%S')
    remainingSec = (oneDayHour - nowTimeJp).total_seconds() + 1   # 1秒加える

    remainingHourStr = '{:.0f}'.format(remainingSec//3600)
    remainingMinStr = '{:.0f}'.format(remainingSec%3600//60)
    remainingTimeStr = '今日の残り時間は、' + remainingHourStr + '時間、' + remainingMinStr + '分です。'

    # 返却文字生成
    dateInfoStr = greetingStr + remainingTimeStr + cheerPhrase

    return dateInfoStr


# 話し言葉を返す
def getAlexSpeakMojiRst():

    dateInfoStr = getDateInfoStr()
    isNesKasa, fukusoMoji = getWeatherInfo()

    # 終わりの言葉
    endSentence = sentences[5][2]
    
    # 傘の要否
    kasaYohi = '不要'
    if isNesKasa:
        kasaYohi = '必要' 
    
    uranaiMoji = getUranaiRst()
    alexSpeakMoji = '{}、さて、今日は、傘は{}で、服装は{}が良いと思います。以下も参考にしましょう。{}。。'\
                    '{}'\
                    .format(dateInfoStr,kasaYohi,fukusoMoji,uranaiMoji,endSentence)
    
    #JSON作成
    alexSpeakJson = [
        {
            'id':1,
            'alexSpeakMoji':alexSpeakMoji,
            'kasaYohi':kasaYohi,
            'fukusoMoji':fukusoMoji
        }
    ]
    
    return alexSpeakJson


@app.route('/recommend/api/what-music/<how_music>', methods=['GET'])
def get_recom_music(how_music):
    recoMusicInfos = getRecoMusicMoji(how_music)
    return flask.jsonify({'recoMusicInfos': recoMusicInfos})

# オススメの楽曲名を返す
def getRecoMusicMoji(how_music):

    recMusicName, predict_val = exeWhatMusic.getMusicName(how_music)

    #JSON作成
    recoMusicInfoJson = [
        {
            'id':1,
            'recoMusicMoji':recMusicName,
            'predict_val':predict_val,
            'how_music':how_music
        }
    ]
    
    return recoMusicInfoJson

# 南郷AIのQA用
@app.route('/nango/api/qa/<question>', methods=['GET'])
def get_nango_qa_answer(question):
    nangoQaInfos = getNangoQaAns(question)
    return flask.jsonify({'nangoQaInfos': nangoQaInfos})

# 南郷AI君のTop画面用のお話用
@app.route('/nango/api/qa/top-comment', methods=['GET'])
def get_top_nango_ai_com():
    topAIcomment = topNangoAICom.get_top_nango_ai_com()
    return flask.jsonify({'topAIcomment': topAIcomment})

# オススメの楽曲名を返す
def getNangoQaAns(question):
    ans_sentence, predict_val = aiNangoQa.getAns(question)
    #JSON作成
    nangoQaInfoJson = [
        {
            'id':1,
            'ans_sentence':ans_sentence,
            'predict_val':predict_val,
            'question':question
        }
    ]
    return nangoQaInfoJson

# メイン関数 Flask起動
if __name__ == "__main__":
    print(" * Flask starting server...")
    app.run(threaded=False, host="0.0.0.0", port=int(os.environ.get("PORT", TM_PORT_NO)))