# css/css-view-transitions/shadow-part-with-name.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/shadow-part-with-name.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: red;
}

::part(party) {
  view-transition-name: party;
}

:root { view-transition-name: none; }
html::view-transition-group(*) { animation-play-state: paused; }
html::view-transition-old(*) { animation: unset; opacity: 0 }
html::view-transition-new(*) { animation: unset; opacity: 0 }
html::view-transition-group(party) {
  position: absolute;
  width: 100px;
  height: 100px;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

      div {
        width: 100px;
        height: 100px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
