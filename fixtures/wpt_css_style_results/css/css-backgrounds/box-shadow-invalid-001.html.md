# css/css-backgrounds/box-shadow-invalid-001.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/box-shadow-invalid-001.html"
}
```

## style[0]

```css

  div
    {
      background-color: red;
      box-shadow: green 100px 100px inset;
      /*
      This inset 'box-shadow' declaration will paint and cover
      the inside of the red square. This way, we verify that
      the UA actually supports 'box-shadow'.
      */
      box-shadow: none, red 0px -100px, red 100px 0px, red 0px 100px, red -100px 0px; /* invalid */
      box-shadow: red 0px -100px, none, red 100px 0px, red 0px 100px, red -100px 0px; /* invalid */
      box-shadow: red 0px -100px, red 100px 0px, none, red 0px 100px, red -100px 0px; /* invalid */
      box-shadow: red 0px -100px, red 100px 0px, red 0px 100px, none, red -100px 0px; /* invalid */
      box-shadow: red 0px -100px, red 100px 0px, red 0px 100px, red -100px 0px, none; /* invalid */
      height: 100px;
      width: 100px;
   }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
