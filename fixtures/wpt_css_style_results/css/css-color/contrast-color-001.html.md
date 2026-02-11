# css/css-color/contrast-color-001.html

```json
{
  "format_version": 3,
  "file": "css/css-color/contrast-color-001.html"
}
```

## style[0]

```css

body {
  background: ivory;
  color: magenta;
}
p {
  font: bold 16px sans-serif;
  padding: .5em;
  background: var(--bgcolor);
  color: contrast-color(var(--bgcolor));
}
/* light colors for which contrast-color() should be black: */
p.test1 {
  --bgcolor: white;
}
p.test2 {
  --bgcolor: aliceblue;
}
p.test3 {
  --bgcolor: mistyrose;
}
p.test4 {
  --bgcolor: lightyellow;
}
p.test5 {
  --bgcolor: palegreen;
}
/* dark colors for which contrast-color() should be white: */
p.test6 {
  --bgcolor: darkblue;
}
p.test7 {
  --bgcolor: maroon;
}
p.test8 {
  --bgcolor: purple;
}
p.test9 {
  --bgcolor: brown;
}
p.test10 {
  --bgcolor: black;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
