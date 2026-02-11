# css/CSS2/tables/separated-border-model-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/separated-border-model-009.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  font: 1em/2 serif;
  margin: 1em;
  }

  div#expected-results {top: 1em;}

  div
  {
  background-color: white;
  height: 3em;
  left: 1em;
  position: absolute;
  width: 600px;
  }

  table
  {
  background-color: red;
  border-collapse: separate;
  border-spacing: 0em 3em;
  margin-top: 0px;
  table-layout: fixed;
  }

  td
  {
  background-color: white;
  height: 2em;
  padding: 0px;
  width: 600px;
  }

  div#between-tbodies {top: 6em;}

  div#after-2nd-tbody {top: 11em;}
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
