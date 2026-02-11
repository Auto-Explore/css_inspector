# css/CSS2/floats-clear/floats-141-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/floats-141-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  p {color: blue;}

  div#wrapper
  {
  border: none 0px;
  display: inline-block;
  margin: 0em;
  padding: 0em;
  }

  div
  {
  border: solid thin;
  margin: 0.2em;
  padding: 0.2em;
  }

  div#A
  {
  color: fuchsia;
  width: 12em;
  }

  div#B
  {
  color: orange;
  display: inline-block;
  width: 8em;
  }

  div#C
  {
  color: orange;
  display: inline-block;
  width: 4em;
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
      "message": "Invalid value for property “color”.",
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
