from utils.aiNangoQa import getAns

def run():
    question = "知識をたくさん付けて選択肢を増やす"
    ans_sentence, predict_val = getAns(question)
    print(ans_sentence, predict_val)