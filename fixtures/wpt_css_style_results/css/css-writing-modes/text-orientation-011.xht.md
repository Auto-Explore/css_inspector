# css/css-writing-modes/text-orientation-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-011.xht"
}
```

## style[0]

```css
<![CDATA[
  @font-face
    {
      font-family: "CSS Full-Width Orientation Test";
      src: url("/fonts/adobe-fonts/CSSFWOrientationTest.otf") format("opentype");
    }

  div
    {
      color: blue;
      font-family: "CSS Full-Width Orientation Test";
      font-size: 180px;
      line-height: 3;
      text-orientation: upright;
      writing-mode: vertical-rl;
    }

  span
    {
      color: orange;
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
      "message": "Unknown property “src”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
