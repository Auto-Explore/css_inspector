# css/css-multicol/multicol-rule-shorthand-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-rule-shorthand-001.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  p
  {
  line-height: 1.25em;
  margin: 1em 0em;
  }

  strong {line-height: 1;}

  div#red-overlapped-reference
  {
  background-color: red;
  height: 100px;
  width: 100px;
  }

  div#test-overlapping-green
  {
  bottom: 100px;
  color: transparent;
  font: 1.25em/1 Ahem; /* equivalent to 20px/1 Ahem */
  position: relative;
  right: 40px;
  width: 9em;

  column-count: 2;
  column-gap: 5em;
  column-rule: green solid 5em;
  column-rule: normal red 5em;
  column-rule: normal 5em red;
  column-rule: red normal 5em;
  column-rule: red 5em normal;
  column-rule: 5em normal red;
  column-rule: 5em red normal;
  column-rule: red 5em red solid;
  column-rule: red 5em solid red;
  column-rule: red solid 5em red;
  column-rule: red solid red 5em;
  column-rule: 5em red solid red;
  column-rule: solid red 5em red;
  }
  ]]>
```

```json
{
  "errors": 15,
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
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
