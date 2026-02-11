# css/CSS2/tables/border-conflict-element-001b.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-001b.xht"
}
```

## style[0]

```css
<![CDATA[
  table {border-collapse: collapse;}

  td
  {
  border-style: solid none solid none;
  border-width: 20px 0px 20px 0px;
  padding: 0px;
  width: 25px;
  }

  td#one {border-color: green red green red;}
  td#two {border-color: green red green red;}
  td#three {border-color: green red green red;}
  td#four {border-color: green red green red;}

  td#five {border-color: red red green red;}
  td#six {border-color: red red green red;}
  td#seven {border-color: red red green red;}
  td#eight {border-color: red red green red;}

  td#nine {border-color: red red green red;}
  td#ten {border-color: red red green red;}
  td#eleven {border-color: red red green red;}
  td#twelve {border-color: red red green red;}

  td#thirteen {border-color: red red green red;}
  td#fourteen {border-color: red red green red;}
  td#fifteen {border-color: red red green red;}
  td#sixteen {border-color: red red green red;}
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
