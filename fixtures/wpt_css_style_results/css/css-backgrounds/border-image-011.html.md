# css/css-backgrounds/border-image-011.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-011.html"
}
```

## style[0]

```css

  div#red-overlapped-ref
    {
      background-color: red;
      padding: 24px;
      height: 52px;
      width: 52px;
      /* border-box is therefore 100px by 100px */
    }

  div#green-overlapping-test
    {
      background-color: green;
      border: red solid 1px;
      height: 50px;
      width: 50px; /* border-box is 52px by 52px */

      border-image-outset: 24px; /* 24px is equal to padding belt */
      border-image-slice: 25;
      border-image-source: url("support/60x60-green.png");
      border-image-width: 25px; /* we make sure that we will fill the outset */
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
