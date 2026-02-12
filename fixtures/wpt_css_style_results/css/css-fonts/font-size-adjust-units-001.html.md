# css/css-fonts/font-size-adjust-units-001.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-size-adjust-units-001.html"
}
```

## style[0]

```css

  div {
    margin: 10px;
    font: 100px/1 Ahem;
    font-size-adjust: 0.8;  /* this matches Ahem's ex-height metric */
    background: orange;
    width: 2ch;
    height: 1ex;
  }
  #test1 {
    background: blue;
    font-size-adjust: 0.4;
    width: 4ch;
    height: 2ex;
  }
  #test2 {
    background: blue;
    font-size-adjust: 1.6;
    width: 1ch;
    height: 0.5ex;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
