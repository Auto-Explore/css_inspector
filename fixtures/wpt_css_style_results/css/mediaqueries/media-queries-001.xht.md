# css/mediaqueries/media-queries-001.xht

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/media-queries-001.xht"
}
```

## style[0]

```css
<![CDATA[
   @media only screen {
     .a, .b { color: green; }
   }
   @media not only screen {
     .b { color: red; }
   }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
