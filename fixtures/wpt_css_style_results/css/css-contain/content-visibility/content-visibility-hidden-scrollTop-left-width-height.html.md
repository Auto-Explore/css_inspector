# css/css-contain/content-visibility/content-visibility-hidden-scrollTop-left-width-height.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-hidden-scrollTop-left-width-height.html"
}
```

## style[0]

```css

body {
  margin: 0;
  padding: 0;
}
#outer {
  width: 200px;
  height: 200px;
  background: lightblue;

  content-visibility: hidden;
}
#inner {
  width: 50px;
  height: 50px;
  background: lightgreen;
  overflow: auto;
}

.content {
  width: 100px;
  height: 100px;
  background-color: red;
}
```

```json
{
  "errors": 2,
  "messages": [
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
