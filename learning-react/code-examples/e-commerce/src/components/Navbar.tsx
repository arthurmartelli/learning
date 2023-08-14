import { useToggle, useWindowSize } from "@react-hookz/web";
import React, { useEffect, useState } from "react";
import { FiMenu } from "react-icons/fi";
import { RiCloseLine } from "react-icons/ri";
import { Link } from "react-router-dom";
import Logo from "./Logo";
import styles from "./Navbar.module.scss";
import { sections } from "@/router";
import { createLinkWithContent } from "@/utils/links";

export default function Navbar() {
  const [isMobile, setIsMobile] = useState(false);
  const size = useWindowSize();

  const items = isMobile ? (
    <HamburgerMenu sections={sections} />
  ) : (
    sections.map((section) => createLinkWithContent(section.name))
  );

  useEffect(() => {
    setIsMobile(size.width <= 600); // Adjust the breakpoint as needed
  }, [size]);

  return (
    <nav className={styles.nav}>
      <Link to="/" key="/">
        <Logo />
      </Link>
      {items}
    </nav>
  );
}

function HamburgerMenu({ sections }: { sections: React.FC[] }) {
  const [menu, toggleMenu] = useToggle(false);
  const openIcon = <RiCloseLine className={styles.icon} onClick={toggleMenu} />;
  const closeIcon = <FiMenu className={styles.icon} onClick={toggleMenu} />;

  return (
    <div className={styles.menu}>
      {menu ? openIcon : closeIcon}

      {menu && (
        <div className={styles.dropdown}>
          {sections.map((section) => createLinkWithContent(section.name))}
        </div>
      )}
    </div>
  );
}
