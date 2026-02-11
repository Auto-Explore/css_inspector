# css/CSS2/floats-clear/margin-collapse-165-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-165-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  td {padding: 0;}

  div.control
  {
  background-color: yellow;
  border: black solid medium;
  width: 7em;
  }

  div.a
  {
  background-color: aqua;
  height: 1.5em;
  margin: 0 0 0 auto;
  width: 4em;
  }

  div.b {height: 0.5em;}

  div.c {background-color: orange;}
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
