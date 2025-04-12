interface TranslateResult {
    src: string;  // 翻译内容
    dst: string;  // 翻译结果
    lang: string; // 检测到的源语言
}

interface TranslateResults {
    trans_result: Array<TranslateResult>
}