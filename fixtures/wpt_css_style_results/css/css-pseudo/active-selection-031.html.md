# css/css-pseudo/active-selection-031.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-031.html"
}
```

## style[0]

```css

  .highlight_reftest {
    overflow: hidden;
    width: 300px;
    height: 325px;
  }
  div > div
    {
      color: red;
      float: left;
      font: 25px/1 Ahem;
      margin-left: 16px;
    }

  div.vrl
    {
      writing-mode: vertical-rl;
    }

  div.vlr
    {
      writing-mode: vertical-lr;
    }

  div.mixed
    {
      text-orientation: mixed;
    }

  div.sideways
    {
      text-orientation: sideways;
    }

  div.upright
    {
      text-orientation: upright;
    }

  div::selection
    {
      background-color: yellow;
      color: green;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
