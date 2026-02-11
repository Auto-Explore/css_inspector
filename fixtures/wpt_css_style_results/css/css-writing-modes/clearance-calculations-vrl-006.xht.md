# css/css-writing-modes/clearance-calculations-vrl-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/clearance-calculations-vrl-006.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      height: 5em; /* computes to 100px */
    }

  div#parent
    {
      background: red url("support/clearance-calculation-vrl-006.png");
      font: 20px/1 Ahem; /* computes to 20px/20px */
      min-width: 7em;
      writing-mode: vertical-rl;
    }

  div > div
    {
      background-color: green;
      width: 1em;
    }

  div#preceding-sibling
    {
      margin-left: 1em;
    }

  div#floated-left
    {
      float: left;
    }

  div#clearing-left /* collapsing-through element */
    {
      clear: left;
      margin-left: 4em;
      margin-right: 5em;
      width: 0em;
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
