# css/css-writing-modes/available-size-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/available-size-001.html"
}
```

## style[0]

```css

body > div {
  font-family: monospace; /* to be able to reliably measure things in ch*/
  font-size: 20px;
  max-height: 8ch; /* **max**-height does not give the element a definite block size */
  overflow: hidden;
  color: transparent;
  position: relative; /* to act as a container of #red */
  padding: 1ch 0;
}

div > div { writing-mode: vertical-rl; }

span {
  background: green;
  display: inline-block; /* This should not change it's size or position, but makes the size of the background predictable*/
}

#red { /* Not necessary when when comparing to the reference, but makes human judgement easier */
  position: absolute;
  background: red;
  left: 0; top: 1ch;
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
