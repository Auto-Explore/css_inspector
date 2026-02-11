# css/css-writing-modes/available-size-017.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/available-size-017.html"
}
```

## style[0]

```css

body > div {
  font-family: monospace; /* to be able to reliably measure things in ch*/
  font-size: 20px;
  height: 4ch;
  min-height: 8ch;
  color: transparent;
  position: relative; /* to act as a container of #green */
}

div > div { writing-mode: vertical-rl; }

span {
  background: green;
  display: inline-block; /* This should not change it's size or position, but makes the size of the background predictable*/
}

#red {
  position: absolute;
  background: red;
  left: 0;
  writing-mode: vertical-rl;
  z-index: -1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
