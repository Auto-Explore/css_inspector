# css/css-backgrounds/border-image-repeat-repeat-001.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-repeat-repeat-001.html"
}
```

## style[0]

```css

  div
    {
      background-color: green; /* so that the center of the + sign gets painted green */
      border: red solid 75px;
      border-image-repeat: repeat; /* this is the same as 'repeat repeat' since
      "
      If the second keyword is absent, it is assumed to be the same as the first.
      https://www.w3.org/TR/css-backgrounds-3/#border-image-repeat
      "
      */
      border-image-source: url("support/500x500-white-red-green-irreg-polygon.png");
      height: 50px;
      width: 50px;
   }

  div#top-plus-sign
    {
      border-image-slice: 50;
      margin-bottom: 10px;
    }

  div#bottom-plus-sign
    {
      border-image-slice: 1;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
