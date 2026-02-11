# css/css-view-transitions/scoped/pause-rendering-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/pause-rendering-ref.html"
}
```

## style[0]

```css


* { box-sizing: border-box; }
div { position: relative; z-index: 0; contain: strict;
      display: inline-block; background: green;
      left: 30px; top: 30px; width: 120px; height: 120px; }
#scope { background: #eee;
         left: 40px; top: 40px; width: 490px; height: 190px; }
#tr { background: orange; left: 60px; }
p { position: absolute; left: 100px; top: 5px; font-size: 30px; }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
