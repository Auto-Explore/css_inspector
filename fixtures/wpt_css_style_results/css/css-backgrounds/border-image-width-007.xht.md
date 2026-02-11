# css/css-backgrounds/border-image-width-007.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-width-007.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      background-color: red;
      border-color: red;
      border-style: none; /* or border-style: dotted or any other 'border-style' value */
      border-width: 0px; /* so the border belt (or border area) of the element is 0px tall and 0px wide */
      border-image-source: url("support/outline-5px-10px-15px-20px-green.png");
      border-image-slice: 5% 10% 15% 20%; /* <number> Percentages are relative to the size of the image: the width of the image for the horizontal offsets, the height for vertical offsets. */
      border-image-width: 50px; /* The four values of ‘border-image-width’ specify offsets that are used to divide the border image area into nine parts. They represent inward distances from the top, right, bottom, and left sides of the area, respectively. */
      border-image-outset: 50px 50px 50px 50px; /* The border-image-outset values specify the amount by which the border image area !_ extends beyond the border box_!. */
      height: 0px;
      margin: 66px 50px 50px 50px;
      /* Why 66px ? In order to take into consideration the fact that the margin-bottom (16px) of the p element will collapse with the top margin of the div element. */
      width: 0px;
    }
  ]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-slice”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-outset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
