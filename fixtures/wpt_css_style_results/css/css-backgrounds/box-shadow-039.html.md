# css/css-backgrounds/box-shadow-039.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/box-shadow-039.html"
}
```

## style[0]

```css

  div
    {
      background-color: rgba(0, 0, 255, 0.5);  /* semi-transparent blue */
      border: black double 18px;
      height: 36px;
      margin-bottom: 36px;
      width: 36px;
    }

  /* Npx 0px, zero spread, not-inset */
  div#sub-test1
    {
      box-shadow: 36px 0px rgba(255, 165, 0, 0.5);  /* semi-transparent orange */
    }

  /* Npx 0px, positive spread, not-inset */
  div#sub-test2
    {
      box-shadow: 36px 0px 0px 18px rgba(255, 165, 0, 0.5);  /* semi-transparent orange */
    }

  /* Npx 0px, negative spread, not-inset */
  div#sub-test3
    {
      box-shadow: 36px 0px 0px -18px rgba(255, 165, 0, 0.5);  /* semi-transparent orange */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
