# css/css-page/parsing/nested-rules-001.html

```json
{
  "format_version": 3,
  "file": "css/css-page/parsing/nested-rules-001.html"
}
```

## style[0]

```css

@page p0{
  @page a{ size: letter; }
}
@page p1{
  @namespace svg url(http://www.w3.org/2000/svg);
}
@page p2{
  @font-face{}
}
@page p3{
  @font-feature-values font one{}
}
@page p4{
  @font-palette-values --alternate{}
}
@page p5{
  @counter-style x{}
}
@page p6{
  @keyframes y{}
}
@page p7{
  @property z{
    syntax: "<color>";
    inherits: false;
    initial-value: #c0ffee;
  }
}
@page p8{
  @import url("style.css") screen;
}
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “syntax”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “inherits”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “initial-value”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
