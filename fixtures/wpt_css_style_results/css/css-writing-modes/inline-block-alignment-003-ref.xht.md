# css/css-writing-modes/inline-block-alignment-003-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/inline-block-alignment-003-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    img
    {
      vertical-align: top;
    }

    img
    {
      padding-left: 210px; /* 60 px (padding-left) + 120px (blue box width) + 30px (the position difference of box) */
    }

    img + br + img
    {
      padding-left: 60px; /* 60 px (padding-left) */
    }

    img + br + img + img
    {
      padding-left: 0;
    }

    img + br + img + img + br + img
    {
      padding-left: 225px; /* 60 px (padding-left) + 120px (blue box width) + 45px (the position difference of box)
    }
  ]]>
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unclosed comment.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
