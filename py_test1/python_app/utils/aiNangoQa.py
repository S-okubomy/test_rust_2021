import pickle
import tensorflow as tf
import MeCab

def inverse_dict(d):
    return {v:k for k,v in d.items()}

# モデルを読み込む
model = tf.keras.models.load_model('python_app/studyModel/qa_nango/hw_model.h5')
# 学習済みの重みデータを読み込む
model.load_weights('python_app/studyModel/qa_nango/hw_weights.h5')
# tfidオブジェクトをファイルよりロードする
vectorizer_loaded = pickle.load(open("python_app/studyModel/qa_nango/tfidf_vec.pickle", "rb"))
# ラベルの定義
labelToCode = pickle.load(open("python_app/studyModel/qa_nango/label_to_code.pickle", "rb"))
label_dic = inverse_dict(labelToCode)
# tagger = MeCab.Tagger('-d /etc/alternatives/mecab-dictionary')
tagger = MeCab.Tagger('mecabrc')

def tokenize(text):
    '''MeCabで形態素解析を行う''' # --- (*3)
    result = []
    word_s = tagger.parse(text)
    # print(word_s)
    for n in word_s.split("\n"):
        if n == 'EOS' or n == '': continue
        p = n.split("\t")[1].split(",")
        h, h2, org = (p[0], p[1], p[6])
        if not (h in ['名詞', '動詞', '形容詞']): continue
        if h == '名詞' and h2 == '数': continue
        if org == '*': org = n.split("\t")[0]
        result.append(org)
    # return result
    return ' '.join(result)

# テキストを指定して判定
def getAns(text):
    # TF-IDFのベクトルに変換 
    data = vectorizer_loaded.transform([tokenize(text)]).toarray()
    # MLPで予測
    pre = model.predict(data)[0]
    sortIndexDesc = pre.argsort()[::-1]
    maxInd = sortIndexDesc[0]
    ans_sentence = label_dic[maxInd]
    predict_val = "{:.4f}".format(pre[maxInd])

    print(ans_sentence, predict_val)
    print("2番目の答え : " + label_dic[sortIndexDesc[1]], "{:.4f}".format(pre[sortIndexDesc[1]]))
    return ans_sentence, predict_val

if __name__ == '__main__':
    requestParam = """
    # 知識をたくさん付けて選択肢を増やす
    コロナ対策教えて？
    """
    getAns(requestParam)