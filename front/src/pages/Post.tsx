import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import ReactMarkdown from "react-markdown";

export default function Post() {
  const { slug } = useParams<{ slug: string }>();
  const [content, setContent] = useState<string>("");

  useEffect(() => {
    if (!slug) return;
    fetch(`/api/post/${slug}`)
      .then((res) => res.json())
      .then(setContent);
  }, [slug]);

  return (
    <main>
      <h1>{slug?.replace("-", " ")}</h1>
      <ReactMarkdown>{content}</ReactMarkdown>
    </main>
  );
}
