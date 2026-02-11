# css/css-writing-modes/abs-pos-border-offset-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-border-offset-001-ref.html"
}
```

## style[0]

```css

  body { margin: 0; }
  .cb {
    position: relative;
    inline-size: 45px;
    block-size: 40px;
    background: lightblue;
    border: solid gray;
    border-width: 1px 2px 3px 4px;
    float: left;
    margin-right: 5px;
  }
  .parent {
    inline-size: 35px;
    block-size: 30px;
    background: orange;
  }
  .abspos {
    inline-size: 20px;
    block-size: 15px;
    background: pink;
  }

  .vrl {
    writing-mode: vertical-rl;
  }
  .vlr {
    writing-mode: vertical-lr;
  }
  .htb {
    writing-mode: horizontal-tb;
  }

  .ltr {
    direction: ltr;
  }
  .rtl {
    direction: rtl;
  }

  .sep {
    clear: both;
    display: block;
    height: 5px;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
