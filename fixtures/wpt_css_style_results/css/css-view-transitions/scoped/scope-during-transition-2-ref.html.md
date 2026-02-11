# css/css-view-transitions/scoped/scope-during-transition-2-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/scope-during-transition-2-ref.html"
}
```

## style[0]

```css

  #outer {
    box-sizing: border-box;
    border: 5px solid black;
    background-color: green;
    height: 300px;
    width: 300px;
    position: relative;
    view-transition-name: outer;
  }

  #inner {
    box-sizing: border-box;
    border: 5px solid black;
    background-color: rebeccapurple;
    height: 100px;
    width: 100px;
    position: absolute;
    left: 100px;
    top: 100px;
    view-transition-name: inner;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
