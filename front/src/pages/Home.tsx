import { useEffect, useState } from "react";
import { Link } from "react-router-dom";

type PostMetadata = {
  slug: string;
  title: string;
};

export default function Home() {
  const [posts, setPosts] = useState<PostMetadata[]>([]);

  useEffect(() => {
    fetch("/api/posts")
      .then((res) => res.json())
      .then((data: PostMetadata[]) => setPosts(data));
  });

  return (
    <main>
      <h1>Cauldrun</h1>
      <ul>
        {posts.map((post) => (
          <li key={post.slug}>
            <Link to={`/post/${post.slug}`}>{post.title}</Link>
          </li>
        ))}
      </ul>
    </main>
  );
}
