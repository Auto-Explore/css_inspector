# css/css-page/page-counters-000.xht

```json
{
  "format_version": 3,
  "file": "css/css-page/page-counters-000.xht"
}
```

## style[0]

```css
<![CDATA[
   	body {counter-reset: chapter;}
	div.chapter {counter-increment: chapter;}
	@page {
	  @top-center { content: "Chapter " counter(chapter); }
	}
	div.section {page-break-before: always;}
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown at-rule.",
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
