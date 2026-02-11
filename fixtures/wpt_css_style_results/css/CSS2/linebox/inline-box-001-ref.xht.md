# css/CSS2/linebox/inline-box-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/inline-box-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div {width: 192px;}

  span {border-color: blue;}

  span#top
  {
  border-style: solid none solid solid;
  border-width: 2px 0px 2px 2px;
  }

  div#middle {background-color: orange;}

  span#bottom
  {
  border-style: solid solid solid none;
  border-width: 2px 2px 2px 0px;
  }
  ]]>
```

```json
{
  "errors": 3,
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
