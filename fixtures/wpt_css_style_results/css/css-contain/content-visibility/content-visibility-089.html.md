# css/css-contain/content-visibility/content-visibility-089.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-089.html"
}
```

## style[0]

```css

body {
  margin: 0;
  padding: 0;
}
#outer {
  width: 100px;
  height: 100px;
  background: lightblue;

  content-visibility: hidden;
}
#inner {
  border-left: 25px black solid;
  border-top: 25px black solid;
  margin: 25px;
  width: 50px;
  height: 50px;
  background: lightgreen;
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
