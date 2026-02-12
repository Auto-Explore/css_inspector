# css/css-contain/contain-content-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-content-002.html"
}
```

## style[0]

```css

  div
    {
      contain: content;
      margin: 30px 0px;
    }

  div#grand-grand-parent-orange
    {
      background-color: orange;
    }

  div#grand-parent-blue
    {
      background-color: blue;
    }

  div#parent-lime
    {
      background-color: lime;
    }

  div#collapse-through-child  /* margin collapsing through element */
    {
      contain: none;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
