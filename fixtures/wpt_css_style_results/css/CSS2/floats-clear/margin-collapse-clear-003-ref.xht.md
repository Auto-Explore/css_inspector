# css/CSS2/floats-clear/margin-collapse-clear-003-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/margin-collapse-clear-003-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  border-collapse: separate;
  border-spacing: 0px;
  }

  td#first-column {padding: 0px 4px 0px 6px;}

  td#second-column
  {
  border: black solid thin;
  padding: 0px;
  width: 150px;
  }

  div {height: 50px;}

  div.sliver-yellow {border-left: yellow solid 10px;}

  div.sliver-aqua
  {
  border-left: 5px solid aqua;
  height: 20px;
  }

  div.mid-container {height: 60px;}

  div.white
  {
  border-left: 5px solid white;
  height: 40px;
  }

  div.sliver-orange {border-left: orange solid 10px;}

  div.yellow {background-color: yellow;}

  div.aqua
  {
  background-color: aqua;
  height: 20px;
  width: 20px;
  }

  div.orange {background-color: orange;}
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
      "message": "Invalid value for property “border”.",
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
