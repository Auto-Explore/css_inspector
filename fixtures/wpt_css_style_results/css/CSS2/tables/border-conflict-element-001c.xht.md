# css/CSS2/tables/border-conflict-element-001c.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-001c.xht"
}
```

## style[0]

```css
<![CDATA[
  table {border-collapse: collapse;}

  td
  {
  border-style: none solid none solid;
  border-width: 0px 20px 0px 20px;
  height: 25px;
  padding: 0px;
  }

  td#one {border-color: red green red green;}
  td#two {border-color: red green red red;}
  td#three {border-color: red green red red;}
  td#four {border-color: red green red red;}

  td#five {border-color: red green red green;}
  td#six {border-color: red green red red;}
  td#seven {border-color: red green red red;}
  td#eight {border-color: red green red red;}

  td#nine {border-color: red green red green;}
  td#ten {border-color: red green red red;}
  td#eleven {border-color: red green red red;}
  td#twelve {border-color: red green red red;}

  td#thirteen {border-color: red green red green;}
  td#fourteen {border-color: red green red red;}
  td#fifteen {border-color: red green red red;}
  td#sixteen {border-color: red green red red;}
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
