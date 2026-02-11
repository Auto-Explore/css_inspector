# css/css-contain/contain-paint-022.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-022.html"
}
```

## style[0]

```css

  div#incorrect-containing-block
    {
      color: transparent;
      font-family: Ahem;
      font-size: 100px;
      height: 2em;
      line-height: 1;
      position: relative;
      width: 2em;
    }

  div#correct-containing-block
    {
      background-color: red;
      contain: paint;
      display: inline-block;
      height: 1em;
      width: 1em;
    }

  div#abspos
    {
      background-color: green;
      height: 1em;
      left: 0;
      position: absolute;
      top: 0;
      width: 1em;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
