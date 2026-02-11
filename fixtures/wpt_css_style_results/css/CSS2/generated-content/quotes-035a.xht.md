# css/CSS2/generated-content/quotes-035a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/quotes-035a.xht"
}
```

## style[0]

```css

  <![CDATA[
     .party1 * { display: inline; }
     .party1 .a { quotes: "Isn"        "'"
                          "t"          "FAIL!"
                          "FAIL!"      " i"; }
     .party1 .b { quotes: ""           "FAIL!!"
                          " wonderful" "!!!"
                          " to "       " work"
                          "see "       " [FAIL to]"
                          "C"          "quotes"
                          "S"          " "
                          "S"          " "; }
     .party1 .c { quotes: none; }
     .party1 .d { quotes: "FAIL!"      "FAIL!"
                          "FAIL!"      "FAIL!"
                          ""           ""
                          ""           ""; }
     .test { margin-left: 2em; }
     .test .no-open:before { content: no-open-quote; }
     .test .open:before { content: open-quote; }
     .test .triple-open:before { content: open-quote open-quote open-quote; }
     .test .no-close:after { content: no-close-quote; }
     .test .triple-no-close:after { content: no-close-quote no-close-quote no-close-quote; }
     .test .close:after { content: close-quote; }
     .test .triple-close:after { content: close-quote close-quote close-quote; }
     .test .no-close-open:before { content: no-close-quote open-quote; }

  ]]>
  
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
