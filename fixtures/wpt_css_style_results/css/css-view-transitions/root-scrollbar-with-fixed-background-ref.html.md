# css/css-view-transitions/root-scrollbar-with-fixed-background-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/root-scrollbar-with-fixed-background-ref.html"
}
```

## style[0]

```css

#container {
  background: lightblue;
}
#first {
  width: 10px;
  background: linear-gradient(green, blue);
  height: 1000px;
}
body {
  margin: 0px;
  padding: 0px;
  filter: invert(1);
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
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
