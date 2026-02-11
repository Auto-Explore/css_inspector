# css/CSS2/linebox/empty-inline-002-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/empty-inline-002-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  p {margin: 1em auto 0em;}

  div
  {
  background-color: green;
  height: 150px;
  margin: 0px 125px 125px;
  padding: 100px 0px;
  width: 250px;
  }

  div > div
  {
  background-color: transparent;
  border: green solid 25px;
  height: 100px;
  padding: 0px;
  position: relative;
  right: 250px;
  width: 500px;
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
