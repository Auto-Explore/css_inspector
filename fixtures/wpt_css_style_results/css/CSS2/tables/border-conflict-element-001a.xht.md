# css/CSS2/tables/border-conflict-element-001a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-element-001a.xht"
}
```

## style[0]

```css
<![CDATA[
  table {border-collapse: collapse;}

  td
  {
  border-style: solid solid solid solid;
  border-width: 20px 20px 20px 20px;
  padding: 0px;
  }

  td#one {border-color: green green green green;}
  td#two {border-color: green green green red;}
  td#three {border-color: green green green red;}
  td#four {border-color: green green green red;}

  td#five {border-color: red green green green;}
  td#six {border-color: red green green red;}
  td#seven {border-color: red green green red;}
  td#eight {border-color: red green green red;}

  td#nine {border-color: red green green green;}
  td#ten {border-color: red green green red;}
  td#eleven {border-color: red green green red;}
  td#twelve {border-color: red green green red;}

  td#thirteen {border-color: red green green green;}
  td#fourteen {border-color: red green green red;}
  td#fifteen {border-color: red green green red;}
  td#sixteen {border-color: red green green red;}
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
