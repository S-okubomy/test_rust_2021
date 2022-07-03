from utils.aiNangoQa import getAns

def run():
    question = "コロナ対策を教えてください"
    ans_sentence, predict_val = getAns(question)
    print(ans_sentence, predict_val)