# css/css-content/content-none-select-1-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-content/content-none-select-1-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:4px/1 Ahem; padding:0; margin:0;
}
body {
  margin-left: 3ch;
}
* { font:10px/1 Ahem; }
.wrapper {
  display: inline-block;
  position: relative;
  height: 2em;
  width: 5em;
  border: 1px solid black;
  margin: 1px;
  vertical-align: top;
}
.wrapper > * {
  padding:0; margin:0; border:none;
}
::marker { content: 'M'; }
.after::after { content: 'A'; }
.before::before { content: 'B'; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
