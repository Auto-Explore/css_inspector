# css/css-contain/contain-strict-003.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-strict-003.html"
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

  div#with-contain-strict
    {
      background-color: orange;
      width: 12em;

      contain: strict;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
