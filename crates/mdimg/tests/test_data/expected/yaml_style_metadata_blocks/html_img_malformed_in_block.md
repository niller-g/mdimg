<div>
Image { span: 6..29, raw: "<img a\"b=c src=one.png>", kind: Html({"a\"b": "c", "src": "one.png"}) }
Image { span: 30..51, raw: "<img =eq src=two.png>", kind: Html({"=eq": "", "src": "two.png"}) }
Image { span: 52..71, raw: "<img/src=three.png>", kind: Html({"src": "three.png"}) }
Image { span: 72..91, raw: "<img//src=four.png>", kind: Html({"src": "four.png"}) }
Image { span: 92..114, raw: "<img \"\"\" src=five.png>", kind: Html({"\"\"\"": "", "src": "five.png"}) }
Image { span: 115..135, raw: "<img src=six.png\"\" >", kind: Html({"src": "six.png\"\""}) }
Image { span: 136..155, raw: "<img src seven.png>", kind: Html({"seven.png": "", "src": ""}) }
<img<img src=eight.png>
</div>