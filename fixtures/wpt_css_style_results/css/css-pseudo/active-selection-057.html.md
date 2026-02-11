# css/css-pseudo/active-selection-057.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/active-selection-057.html"
}
```

## style[0]

```css

  div#subtest1 , hr#subtest2
    {
      background-color: transparent; /* or initial or unset */
      height: 100px;
    }

  hr#subtest2
    {
      border: none 0px;
    }

  div#subtest1::selection , hr#subtest2::selection
    {
      color: red;
    }

  div#subtest3
    {
      background-color: transparent; /* or initial or unset */
      font-size: 100px;
      white-space: pre; /* or pre-line or pre-wrap or break-spaces */
    }

  div#subtest3::selection
    {
      background-color: red;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
