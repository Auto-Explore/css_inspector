# css/css-color/color-mix-percents-02.html

```json
{
  "format_version": 3,
  "file": "css/css-color/color-mix-percents-02.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 14em; height: 2em; margin-top: 0; margin-bottom: 0;}
    .negative-test { background-color: rgb(68.4898% 36.015% 68.3102%); width: 14em; height: 2em; margin-top: 0; margin-bottom: 0;}
    .t1 { background-color: rgb(68.4898% 36.015% 68.3102%); }
    .t2 { background-color: color-mix(in lch, purple 50%, plum 50%);}
    .t3 { background-color: color-mix(in lch, purple 55%, plum 55%);}
    .t4 { background-color: color-mix(in lch, purple 70%, plum 70%); }
    .t5 { background-color: color-mix(in lch, purple 95%, plum 95%);}
    .t6 { background-color: color-mix(in lch, purple 125%, plum 125%);}
    .t7 { background-color: color-mix(in lch, purple 9999%, plum 9999%);}
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
