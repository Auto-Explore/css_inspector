# css/css-contain/contain-layout-ink-overflow-013.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-ink-overflow-013.html"
}
```

## style[0]

```css

  div#outer
    {
      font-family: monospace;
      font-size: 100px;
      height: 3em;
      line-height: 1.5; /* computes to 150px */
      width: 4ch;

      overflow: scroll;
    }

  div#inner
    {
      color: red;
      contain: layout;
      width: 0;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
