# css/css-values/angle-units-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/angle-units-001.html"
}
```

## style[0]

```css

  div
    {
      height: 100px;
      width: 100px;
    }

  div#test-overlapping-green
    {
      background-image: linear-gradient(green, green);
      background-image: linear-gradient(90degree, red, red);   /* invalid; 90deg is valid */
      background-image: linear-gradient(100gradian, red, red); /* invalid; 100grad is valid */
      background-image: linear-gradient(1.57radian, red, red); /* invalid; 1.57rad is valid */
      background-image: linear-gradient(0.25turns, red, red);  /* invalid; 0.25turn is valid */
    }

  div#reference-overlapped-red
    {
      background-color: red;
      bottom: 100px;
      position: relative;
      z-index: -1;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
