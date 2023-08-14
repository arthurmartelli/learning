import { Link } from "react-router-dom";

export function createLinkWithContent(section: string) {
  const name = section.toLowerCase();
  return (
    <Link key={name} to={`/${name}`}>
      {name}
    </Link>
  );
}
