# css/selectors/old-tests/css3-modsel-176.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-176.xml"
}
```

## style[0]

```css
<![CDATA[
p { background: red; color: yellow; }
p:not(#other).class:not(.fail).test#id#id { background: green; color: white; }
div { background: green; color: white; }
div:not(#theid).class:not(.fail).test#theid#theid { background: red; color: yellow; }
div:not(#other).notclass:not(.fail).test#theid#theid { background: red; color: yellow; }
div:not(#other).class:not(.test).test#theid#theid { background: red; color: yellow; }
div:not(#other).class:not(.fail).nottest#theid#theid { background: red; color: yellow; }
div:not(#other).class:not(.fail).nottest#theid#other { background: red; color: yellow; }
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
