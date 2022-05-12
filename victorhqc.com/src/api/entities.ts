export type Picture = {
  id: string;
  width: number;
  height: number;
  color: string | null;
  blur_hash: string | null;
  downloads: number;
  likes: number;
  description: string | null;
  alt_description: string | null;
  location: Location;
  exif: Exif;
  links: Links;
};

type Links = {
  self: string;
  html: string;
  download: string;
  download_location: string;
};

type Location = {
  name: string | null;
  city: string | null;
  country: string | null;
};

type Exif = {
  make: string | null;
  model: string | null;
  exposure_time: string | null;
  aperture: string | null;
  focal_length: string | null;
  iso: number | null;
};

type User = {
  id: string;
  username: string;
  name: string;
  portfolio_url: string | null;
  bio: string | null;
  location: string | null;
  instagram_username: string | null;
  twitter_username: string | null;
  links: UserLinks;
};

type UserLinks = {
  self: string;
  html: string;
  photos: string;
  likes: string;
  portfolio: string;
};
