# css/css-contain/contain-paint-size-003.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-size-003.html"
}
```

## style[0]

```css

  div
    {
      color: transparent;
      font-size: 16px;
      padding: 8px;
    }

  div#floated-left
    {
      background-color: blue;
      float: left;
      margin: 8px;
      width: 6em;
    }

  div#with-contain-paint-size
    {
      background-color: orange;
      width: 12em;

      contain: paint size;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
