# css/css-backgrounds/border-image-width-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-width-005.xht"
}
```

## style[0]

```css
<![CDATA[
  p
    {
      margin-bottom: 41px; /* border-image-outset + 16px (1em) .
      This 41px margin-bottom value is just to position the rendered layout to accurately match the reference file . */
    }

  div
    {
      background-color: red;
      border-color: red;
      border-style: none; /* or border-style: dotted or any other 'border-style' value */
      border-width: 0px; /* so the border belt (or border area) of the element is 0px tall and 0px wide */
      border-image-source: url("support/outline-5px-10px-15px-20px-green.png");
      border-image-slice: 5% 10% 15% 20%; /* <number> Percentages are relative to the size of the image: the width of the image for the horizontal offsets, the height for vertical offsets. */
      border-image-width: 50px; /* The four values of ‘border-image-width’ specify offsets that are used to divide the border image area into nine parts. They represent inward distances from the top, right, bottom, and left sides of the area, respectively. */
      border-image-outset: 25px 25px 25px 25px; /* The border-image-outset values specify the amount by which the border image area *_ extends beyond the border box_*. */
      height: 20px;
      margin-left: 25px; /* == border-image-outset . This margin-left value is just to position the rendered layout to accurately match the reference file . */
      padding: 15px 15px 15px 15px;
      width: 20px;
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
