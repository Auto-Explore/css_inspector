# css/CSS2/fonts/font-family-rule-002a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-family-rule-002a.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  font-size: 1.25em;
  line-height: 1.5;
  }

  #quoted
  {
  font-family: "White Space";
  }

  #unquoted
  {
  font-family: White Space;
  }

  #unquoted-spaces-before
  {
  font-family:        White Space;
  }

  #unquoted-spaces-between
  {
  font-family: White       Space;
  }

  #unquoted-spaces-after
  {
  font-family: White Space       ;
  }

  #unquoted-spaces-all
  {
  font-family:        White        Space       ;
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
