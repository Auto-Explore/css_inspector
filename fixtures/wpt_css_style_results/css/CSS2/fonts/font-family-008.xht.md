# css/CSS2/fonts/font-family-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/fonts/font-family-008.xht"
}
```

## style[0]

```css
<![CDATA[
  p#first-test {font-family: Ahem, foo(bar), sans-serif;}

  p#second-test {font-family: Ahem, foo(bar)foo, sans-serif;}

  p#third-test {font-family: Ahem, foo{bar}, sans-serif;}

  p#fourth-test {font-family: Ahem, foo{bar}foo, sans-serif;}
  ]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-family”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
